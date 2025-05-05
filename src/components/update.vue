<template>
  <div class="update-container">
    <div class="update-section">
      <button @click="checkUpdate" :disabled="isUpdating">检查更新</button>
      <div v-if="updateMessage" class="update-message">{{ updateMessage }}</div>
      <div v-if="showProgress" class="progress-container">
        <div class="progress-bar" :style="{ width: progressPercentage + '%' }"></div>
        <div class="progress-text">{{ progressPercentage }}%</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Update'})
  import { ref } from 'vue';
  import { check } from '@tauri-apps/plugin-updater';
  import { relaunch } from '@tauri-apps/plugin-process';

  const updateMessage = ref('');
  const isUpdating = ref(false);
  const showProgress = ref(false);
  const progressPercentage = ref(0);

  // 更新功能
  const checkUpdate = async () => {
    try {
      isUpdating.value = true;
      updateMessage.value = '正在检查更新...';
      
      const update = await check();
      
      if (!update) {
        updateMessage.value = '您已经在使用最新版本';
        setTimeout(() => {
          updateMessage.value = '';
          isUpdating.value = false;
        }, 3000);
        return;
      }
      
      updateMessage.value = `发现新版本 ${update.version}，更新说明: ${update.body}`;
      showProgress.value = true;
      
      let downloaded = 0;
      let contentLength = 0;
      
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength!;
            updateMessage.value = `开始下载更新，总大小: ${(contentLength / 1024 / 1024).toFixed(2)}MB`;
            break;
          case 'Progress':
            downloaded += event.data.chunkLength;
            progressPercentage.value = Math.floor((downloaded / contentLength) * 100);
            break;
          case 'Finished':
            updateMessage.value = '下载完成，准备安装...';
            break;
        }
      });

      updateMessage.value = '更新已安装，即将重启应用...';
      setTimeout(async () => {
        await relaunch();
      }, 2000);
    } catch (error) {
      console.error('Update error:', error);
      updateMessage.value = `更新失败: ${error}`;
      isUpdating.value = false;
      showProgress.value = false;
    }
  }
</script>

<style scoped>
.update-container {
  .update-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;
  }

  button {
  padding: 10px 20px;
  border-radius: 6px;
  background-color: #4CAF50;
  color: white;
  border: none;
  cursor: pointer;
  font-size: 16px;
  transition: background-color 0.3s;
  }

  button:hover:not(:disabled) {
  background-color: #45a049;
  }

  button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
  }

  .update-message {
    margin-top: 10px;
    padding: 10px;
    border-radius: 4px;
    background-color: #f0f0f0;
    color: #333;
  }

  .progress-container {
    width: 100%;
    height: 20px;
    background-color: #e0e0e0;
    border-radius: 10px;
    overflow: hidden;
    position: relative;
  }

  .progress-bar {
    height: 100%;
    background-color: #4CAF50;
    transition: width 0.3s ease;
  }

  .progress-text {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #000;
    font-weight: bold;
  }
}
</style>
 