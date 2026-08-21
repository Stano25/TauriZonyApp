import { invoke } from '@tauri-apps/api/core';
import { appSettings } from '$lib/settings.svelte';
import { appendTerminalLog, clearTerminalLogs } from '$lib/terminal';

type ProtocolType = 'lte' | '5g' | 'db';

type ProtocolGenerationArgs = {
  protocolType: ProtocolType;
  protocolPath: string;
  gsmPath: string;
  ltePath: string;
  fivegPath?: string;
  outputPath: string;
  measuredCity: string;
  totalPower: string;
  sinr: string;
  rsrp: string;
  antennaHeight: string;
  internalEnvironment: string;
  rsrpMode?: string;
  sinrMode?: string;
};

function parseNumberOrThrow(value: string, label: string): number {
  const normalized = value.replace(',', '.').trim();
  const parsed = Number(normalized);
  if (!Number.isFinite(parsed)) {
    throw new Error(`Neplatna ciselna hodnota pre ${label}: ${value}`);
  }
  return parsed;
}

export async function runProtocolGeneration(args: ProtocolGenerationArgs) {
  if (appSettings.is_generating) {
    await appendTerminalLog('Generovanie uz prebieha.', 'warning');
    return;
  }

  const missing: string[] = [];

  if (!args.protocolPath.trim()) {
    missing.push('Sablona protokolu (XLSX)');
  }
  if (!args.gsmPath.trim()) {
    missing.push('GSM CSV subor');
  }
  if (!args.ltePath.trim()) {
    missing.push('LTE CSV subor');
  }
  if (!args.outputPath.trim()) {
    missing.push('Vystupny protokol (XLSX)');
  }
  if ((args.protocolType === '5g' || args.protocolType === 'db') && !args.fivegPath?.trim()) {
    missing.push('5G CSV subor');
  }

  const selectedRsrpMode = args.rsrpMode?.trim() || appSettings.fiveg_rsrp?.trim();
  const selectedSinrMode = args.sinrMode?.trim() || appSettings.fiveg_sinr?.trim();

  if ((args.protocolType === '5g' || args.protocolType === 'db') && !selectedRsrpMode) {
    missing.push('Vyber 5G RSRP stlpca');
  }

  if ((args.protocolType === '5g' || args.protocolType === 'db') && !selectedSinrMode) {
    missing.push('Vyber 5G SINR stlpca');
  }

  if (missing.length > 0) {
    await clearTerminalLogs();
    await appendTerminalLog(`Chybaju povinne vstupy: ${missing.join(', ')}`, 'error');
    return;
  }

  try {
    const totalPower = parseNumberOrThrow(args.totalPower, 'totalPower');
    const sinr = parseNumberOrThrow(args.sinr, 'sinr');
    const rsrp = parseNumberOrThrow(args.rsrp, 'rsrp');
    const antennaHeight = parseNumberOrThrow(args.antennaHeight, 'antennaHeight');
    const internalEnvironment = parseNumberOrThrow(args.internalEnvironment, 'internalEnvironment');

    await clearTerminalLogs();
    appSettings.is_generating = true;

    await appendTerminalLog(`Spusta sa generovanie protokolu (${args.protocolType.toUpperCase()})...`, 'normal');

    const resultMessage = await invoke<string>('generate_protocol_data', {
      protocolType: args.protocolType,
      protocolPath: args.protocolPath,
      gsmPath: args.gsmPath,
      ltePath: args.ltePath,
      fivegPath: args.protocolType === 'lte' ? null : args.fivegPath ?? null,
      outputPath: args.outputPath,
      measuredCity: args.measuredCity,
      totalPower,
      sinr,
      rsrp,
      antennaHeight,
      internalEnvironment,
      rsrpMode: args.protocolType === 'lte' ? null : (selectedRsrpMode ?? null),
      sinrMode: args.protocolType === 'lte' ? null : (selectedSinrMode ?? null),
    });

    await appendTerminalLog('Protokol bol uspesne vygenerovany.', 'success');
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
