import { emit } from '@tauri-apps/api/event';

export type TerminalLogType = 'normal' | 'success' | 'warning' | 'error';

export async function appendTerminalLog(message: string, logType: TerminalLogType = 'normal') {
  await emit('terminal-log', {
    message,
    log_type: logType,
  });
}

export async function clearTerminalLogs() {
  await emit('terminal-clear');
}
