import { invoke } from '@tauri-apps/api/core';
import { appSettings } from '$lib/settings.svelte';
import { appendTerminalLog, clearTerminalLogs } from '$lib/terminal';

type ZonyDataType = 'gsm' | 'lte' | '5g';

type RunZonyGenerationArgs = {
  dataType: ZonyDataType;
  inputPath: string;
  outputPath: string;
  rsrpMode?: string;
  sinrMode?: string;
  onlyBestOperator: boolean;
  minBestOperatorSinr: string;
};

function parseNumberOrThrow(value: string, label: string): number {
  const normalized = value.replace(',', '.').trim();
  const parsed = Number(normalized);
  if (!Number.isFinite(parsed)) {
    throw new Error(`Neplatna ciselna hodnota pre ${label}: ${value}`);
  }
  return parsed;
}

function getMissingFields(args: RunZonyGenerationArgs): string[] {
  const missing: string[] = [];
  const useFilter = args.dataType !== 'gsm' && appSettings.use_filter;
  // Prah ma zmysel len ked je zapnuty rezim najlepsieho operatora a sila ma SINR (LTE/5G)
  const needsSinrThreshold = args.onlyBestOperator && args.dataType !== 'gsm';

  if (!appSettings.zone_file_path?.trim()) {
    missing.push('Subor definovanych zon');
  }
  if (!args.inputPath?.trim()) {
    missing.push('Vstupny CSV subor');
  }
  if (!args.outputPath?.trim()) {
    missing.push('Vystupny CSV subor');
  }
  if (useFilter && !appSettings.filter_path?.trim()) {
    missing.push('Priecinok filtrov');
  }
  if (args.dataType === '5g' && !args.rsrpMode?.trim()) {
    missing.push('Vyber RSRP stlpca');
  }
  if (args.dataType === '5g' && !args.sinrMode?.trim()) {
    missing.push('Vyber SINR stlpca');
  }
  if (needsSinrThreshold && !args.minBestOperatorSinr?.trim()) {
    missing.push('Minimalny SINR prah najlepsieho operatora');
  }

  return missing;
}

export async function runZonyGeneration(args: RunZonyGenerationArgs) {
  if (appSettings.is_generating) {
    await appendTerminalLog('Generovanie uz prebieha.', 'warning');
    return;
  }

  const missing = getMissingFields(args);
  if (missing.length > 0) {
    await clearTerminalLogs();
    await appendTerminalLog(`Chybaju povinne vstupy: ${missing.join(', ')}`, 'error');
    return;
  }

  // Prah sa pouziva len pri LTE a 5G (GSM nema SINR)
  const needsSinrThreshold = args.onlyBestOperator && args.dataType !== 'gsm';

  try {
    const bestSinrThreshold = needsSinrThreshold
      ? parseNumberOrThrow(args.minBestOperatorSinr, 'minimalny SINR prah')
      : null;

    await clearTerminalLogs();
    appSettings.is_generating = true;
    const useFilter = args.dataType !== 'gsm' && appSettings.use_filter;

    await appendTerminalLog(`Spusta sa generovanie dat pre ${args.dataType.toUpperCase()}...`, 'normal');
    if (args.onlyBestOperator) {
      await appendTerminalLog('Rezim: pre kazdu zonu bude ponechany iba najlepsi operator.', 'normal');
    }

    const resultMessage = await invoke<string>('generate_zony_data', {
      dataType: args.dataType,
      gridPath: appSettings.zone_file_path,
      inputPath: args.inputPath,
      outputPath: args.outputPath,
      generateMissingOperators: args.onlyBestOperator ? false : appSettings.generate_missing_operators,
      onlyBestOperator: args.onlyBestOperator,
      minBestOperatorSinr: bestSinrThreshold,
      useFilter: useFilter,
      filterPath: useFilter ? appSettings.filter_path || '' : '',
      createDuplicates: useFilter ? appSettings.create_duplicates : false,
      rsrpMode: args.dataType === '5g' ? args.rsrpMode ?? null : null,
      sinrMode: args.dataType === '5g' ? args.sinrMode ?? null : null,
    });

    await appendTerminalLog('Data boli uspesne vygenerovane.', 'success');
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
