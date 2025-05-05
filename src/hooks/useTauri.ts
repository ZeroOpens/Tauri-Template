import { ref, onMounted } from 'vue';
import { getTauriVersion, getName, getVersion } from '@tauri-apps/api/app';

export default function () {
  let tauriVersion = ref('')
  let appName = ref('')
  let appVersion = ref('')

  onMounted( async () => {
    tauriVersion.value = await getTauriVersion();  // 获取tauri版本
    appName.value = await getName();  // 获取应用程序名称
    appVersion.value = await getVersion();  // 获取应用程序版本
  })

  // 导出
  return {tauriVersion, appName, appVersion}
}

