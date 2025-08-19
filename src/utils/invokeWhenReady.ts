import { invoke } from '@tauri-apps/api/core'

export async function invokeWhenReady<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    if ((window as any).__TAURI_IPC__) {
        return invoke<T>(cmd, args)
    }

    await new Promise<void>((resolve) => {
        const check = () => {
            if ((window as any).__TAURI_IPC__) {
                resolve()
            } else {
                requestAnimationFrame(check)
            }
        }
        check()
    })

    return invoke<T>(cmd, args)
}