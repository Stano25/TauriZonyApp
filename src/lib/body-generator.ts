import { invoke } from '@tauri-apps/api/core';
import { appSettings } from '$lib/settings.svelte';
import { appendTerminalLog, clearTerminalLogs } from '$lib/terminal';

type BodyDataType = 'lte' | '5g';

type MobilePathEntry = {
  lte_path: string;
  g5_path: string;
};

type CommonBodyArgs = {
  outputPath: string;
  filterPath: string;
  protocolPath: string;
  secondOutputPath: string;
  useFilter: boolean;
  maxDistance: string;
  generateMissingOperators: boolean;
  thresholdSinr: string;
  thresholdRsrp: string;
  createDuplicates: boolean;
  useProtocolPoints: boolean;
};

type BodyGenerationArgs = CommonBodyArgs & {
  dataType: BodyDataType;
  inputPaths: string[];
};

type MobileBodyGenerationArgs = CommonBodyArgs & {
  inputEntries: MobilePathEntry[];
};

function parseNumberOrThrow(value: string, label: string): number {
  const normalized = value.replace(',', '.').trim();
  const parsed = Number(normalized);
  if (!Number.isFinite(parsed)) {
    throw new Error(`Neplatna ciselna hodnota pre ${label}: ${value}`);
  }
  return parsed;
}

function collectCommonMissing(args: CommonBodyArgs): string[] {
  const missing: string[] = [];

  if (!args.outputPath.trim()) {
    missing.push('Vystupny CSV subor');
  }

  if (args.useFilter && !args.filterPath.trim()) {
    missing.push('Priecinok filtrov');
  }

  if (args.useProtocolPoints && !args.protocolPath.trim()) {
    missing.push('Sablona protokolu (XLSX)');
  }

  if (args.useProtocolPoints && !args.secondOutputPath.trim()) {
    missing.push('Vystupny protokol (XLSX)');
  }

  return missing;
}

export async function runBodyGeneration(args: BodyGenerationArgs) {
  if (appSettings.is_generating) {
    await appendTerminalLog('Generovanie uz prebieha.', 'warning');
    return;
  }

  const missing = collectCommonMissing(args);
  if (args.inputPaths.length === 0) {
    missing.push('Aspoň 1 vstupny CSV subor');
  }

  if (missing.length > 0) {
    await clearTerminalLogs();
    await appendTerminalLog(`Chybaju povinne vstupy: ${missing.join(', ')}`, 'error');
    return;
  }

  try {
    const maxDistance = parseNumberOrThrow(args.maxDistance, 'maxDistance');
    const thresholdSinr = parseNumberOrThrow(args.thresholdSinr, 'thresholdSinr');
    const thresholdRsrp = parseNumberOrThrow(args.thresholdRsrp, 'thresholdRsrp');

    await clearTerminalLogs();
    appSettings.is_generating = true;

    await appendTerminalLog(`Spusta sa generovanie bodov pre ${args.dataType.toUpperCase()}...`, 'normal');

    const resultMessage = await invoke<string>('generate_body_data', {
      dataType: args.dataType,
      inputPaths: args.inputPaths,
      outputPath: args.outputPath,
      filterPath: args.useFilter ? args.filterPath : '',
      protocolPath: args.useProtocolPoints ? args.protocolPath : null,
      secondOutputPath: args.useProtocolPoints ? args.secondOutputPath : '',
      useFilter: args.useFilter,
      maxDistance,
      generateMissingOperators: args.generateMissingOperators,
      thresholdSinr,
      thresholdRsrp,
      createDuplicates: args.useFilter ? args.createDuplicates : false,
      rsrpMode: args.dataType === '5g' ? appSettings.fiveg_rsrp : null,
      sinrMode: args.dataType === '5g' ? appSettings.fiveg_sinr : null,
    });

    await appendTerminalLog('Body boli uspesne vygenerovane.', 'success');
    if (resultMessage) {
      await appendTerminalLog(resultMessage, 'success');
    }
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    await appendTerminalLog(`Generovanie zlyhalo: ${message}`, 'error');
  } finally {
    await appendTerminalLog('Generovanie ukoncene.', 'normal');
    appSettings.is_generating = false;
  }
}

export async function runMobileBodyGeneration(args: MobileBodyGenerationArgs) {
  if (appSettings.is_generating) {
    await appendTerminalLog('Generovanie uz prebieha.', 'warning');
    return;
  }

  const missing = collectCommonMissing(args);

  if (args.inputEntries.length === 0) {
    missing.push('Aspoň 1 NSA 5G bod (LTE + 5G)');
  }

  const invalidEntry = args.inputEntries.find((entry) => !entry.lte_path.trim() || !entry.g5_path.trim());
  if (invalidEntry) {
    missing.push('Kazdy NSA 5G bod musi mat LTE aj 5G cestu');
  }

  if (missing.length > 0) {
    await clearTerminalLogs();
    await appendTerminalLog(`Chybaju povinne vstupy: ${missing.join(', ')}`, 'error');
    return;
  }

  try {
    const maxDistance = parseNumberOrThrow(args.maxDistance, 'maxDistance');
    const thresholdSinr = parseNumberOrThrow(args.thresholdSinr, 'thresholdSinr');
    const thresholdRsrp = parseNumberOrThrow(args.thresholdRsrp, 'thresholdRsrp');

    await clearTerminalLogs();
    appSettings.is_generating = true;

    await appendTerminalLog('Spusta sa generovanie NSA 5G bodov...', 'normal');

    const resultMessage = await invoke<string>('generate_mobile_body_data', {
      inputEntries: args.inputEntries,
      outputPath: args.outputPath,
      filterPath: args.useFilter ? args.filterPath : '',
      protocolPath: args.useProtocolPoints ? args.protocolPath : null,
      secondOutputPath: args.useProtocolPoints ? args.secondOutputPath : '',
      useFilter: args.useFilter,
      maxDistance,
      generateMissingOperators: args.generateMissingOperators,
      thresholdSinr,
      thresholdRsrp,
      createDuplicates: args.useFilter ? args.createDuplicates : false,
      rsrpMode: appSettings.fiveg_rsrp,
      sinrMode: appSettings.fiveg_sinr,
    });

    await appendTerminalLog('NSA 5G body boli uspesne vygenerovane.', 'success');
    if (resultMessage) {
      await appendTerminalLog(resultMessage, 'success');
    }
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    await appendTerminalLog(`Generovanie zlyhalo: ${message}`, 'error');
  } finally {
    await appendTerminalLog('Generovanie ukoncene.', 'normal');
    appSettings.is_generating = false;
  }
}
