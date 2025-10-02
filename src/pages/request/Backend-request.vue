<template>
  <div class="request">
    <div class="request-card">
      <h2>使用 reqwest 请求</h2>
      <!-- 返回按钮 -->
      <button_back @click="$router.back()" class="back-button"></button_back>
      
      <div class="button-group">
        <button 
          @click="getDogImages" 
          :disabled="loadingDog"
          class="request-button dog-button"
        >
          <span v-if="loadingDog" class="loading-spinner"></span>
          <span>{{ loadingDog ? '获取中...' : '获取图片' }}</span>
        </button>
        
        <button 
          @click="getWord" 
          :disabled="loadingWord"
          class="request-button word-button"
        >
          <span v-if="loadingWord" class="loading-spinner"></span>
          <span>{{ loadingWord ? '获取中...' : '获取语句' }}</span>
        </button>
      </div>
      
      <div class="result-container">
        <!-- 语句结果 -->
        <div class="result-item" v-if="word">
          <h3>随机语句</h3>
          <div class="word-result">{{ word }}</div>
        </div>
        
        <!-- 图片结果 -->
        <div class="result-item" v-if="dogImage">
          <h3>随机狗狗图片</h3>
          <div class="image-container">
            <img :src="dogImage" alt="Random Dog Image">
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'BackendRequest'})
  import {ref} from 'vue'
  import { invoke } from '@tauri-apps/api/core';
  import button_back from '@/components/buttons/button_back.vue'


    // 响应式数据
  const dogImage = ref<string>('');
  const loadingDog = ref<boolean>(false);
  const error = ref<string>('');

  // 获取狗狗图片的方法
  const getDogImages = async () => {
    loadingDog.value = true;
    error.value = '';
    dogImage.value = '';
    
    try {
      // 调用Rust后端的get_random_dog_image命令
      const imageUrl = await invoke<string>('get_random_dog_image');
      dogImage.value = imageUrl;
    } catch (err) {
      error.value = `获取图片失败: ${err}`;
      console.error('调用Rust命令失败:', err);
    } finally {
      loadingDog.value = false;
    }
  };


  // 单条语录相关数据
  const word = ref<string>('');
  const loadingWord = ref<boolean>(false);
  const singleError = ref<string>('');

  // 获取单条动漫一言
  const getWord = async () => {
    loadingWord.value = true;
    singleError.value = '';
    word.value = '';
    
    try {
      // 调用Rust后端的get_random_anime_quote命令
      const response = await invoke<{ quote: string }>('get_random_anime_quote');
      word.value = response.quote;
    } catch (err) {
      singleError.value = `获取动漫一言失败: ${err}`;
      console.error('调用Rust命令失败:', err);
    } finally {
      loadingWord.value = false;
    }
  };

  // 多条语录相关数据
  // const animeQuotes = ref<Array<{ quote: string }>>([]);
  // const loadingMultiple = ref<boolean>(false);
  // const multipleError = ref<string>('');
  
  // // 获取多条动漫一言
  // const getWords = async (count: number) => {
  //   loadingMultiple.value = true;
  //   multipleError.value = '';
  //   animeQuotes.value = [];
    
  //   try {
  //     // 调用Rust后端的get_multiple_anime_quotes命令
  //     const quotes = await invoke<Array<{ quote: string }>>('get_multiple_anime_quotes', { count });
  //     animeQuotes.value = quotes;
  //   } catch (err) {
  //     multipleError.value = `获取多条动漫一言失败: ${err}`;
  //     console.error('调用Rust命令失败:', err);
  //   } finally {
  //     loadingMultiple.value = false;
  //   }
  // };
</script>

<style scoped lang="scss">
.request {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  
  .request-card {
    position: relative;
    background: white;
    border-radius: 16px;
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.1);
    width: 100%;
    max-width: 800px;
    
    h2 {
      text-align: center;
      margin-top: 0;
      color: #333;
      font-size: 1.8rem;
      position: relative;
      padding-bottom: 10px;
      
      &:after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 15%;
        height: 3px;
        background: #F44B01;
        border-radius: 3px;
      }
    }

    // 返回按钮
    .back-button {
      position: absolute;
      top: 0px;
      left: 0px;
      border-radius: 16px 0 80% 0;
      
      &:hover {
        border-radius: 16px 0;
      }
    }
  }
  
  .button-group {
    display: flex;
    justify-content: center;
    gap: 20px;
    margin-bottom: 30px;
    flex-wrap: wrap;
  }
  
  .request-button {
    position: relative;
    padding: 12px 25px;
    border-radius: 25px;
    color: white;
    border: none;
    cursor: pointer;
    font-size: 16px;
    font-weight: 600;
    transition: all 0.3s ease;
    min-width: 140px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    
    &:disabled {
      opacity: 0.7;
      cursor: not-allowed;
    }
    
    &:not(:disabled):hover {
      transform: translateY(-3px);
      box-shadow: 0 6px 15px rgba(0, 0, 0, 0.2);
    }
    
    &:not(:disabled):active {
      transform: translateY(-1px);
    }
  }
  
  .dog-button {
    background: linear-gradient(90deg, #4CAF50, #8BC34A);
    box-shadow: 0 4px 10px rgba(76, 175, 80, 0.3);
  }
  
  .word-button {
    background: linear-gradient(90deg, #2196F3, #03A9F4);
    box-shadow: 0 4px 10px rgba(33, 150, 243, 0.3);
  }
  
  // 加载动画
  .loading-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 50%;
    border-top-color: white;
    animation: spin 0.8s linear infinite;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .result-container {
    display: flex;
    flex-direction: column;
    gap: 25px;
  }
  
  .result-item {
    background: #f8f9fa;
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
    
    h3 {
      margin-top: 0;
      margin-bottom: 15px;
      color: #444;
      font-size: 1.3rem;
    }
  }
  
  .word-result {
    font-size: 18px;
    line-height: 1.6;
    color: #333;
    padding: 15px;
    background: white;
    border-radius: 8px;
    border-left: 4px solid #2196F3;
  }
  
  .image-container {
    display: flex;
    justify-content: center;
    
    img {
      max-width: 100%;
      border-radius: 8px;
      box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
      transition: transform 0.3s ease;
      
      &:hover {
        transform: scale(1.02);
      }
    }
  }
}
</style>
 