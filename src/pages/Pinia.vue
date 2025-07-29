<template>
  <div class="pinia-content">
    <!-- Pinia 数据操作区 -->
    <div class="card piniaStorage">
      <h3>Pinia 存储插件</h3>
      
      <div class="input-group">
        <input
          type="text"
          v-model="indexStore.useName"
          placeholder="请输入数据，并在300ms后自动更新本地存储数据"
          @input="handleInput"
        />
        <button class="clear-btn" @click="clearpinia">清空存储</button>
      </div>
    </div>
    
    <!-- LocalStorage 数据展示区 -->
    <div class="card localStorage">
      <h3>本地存储</h3>
      <div class="storage-list">
        <div v-if="Object.keys(parsedData).length === 0" class="empty-data">
          暂无存储数据
        </div>
        <ul v-else>
          <li v-for="(value, key) in parsedData" :key="key" class="storage-item">
            <div class="storage-key">持久化数据:</div>
            <div class="storage-value">{{ typeof value === 'object' ? JSON.stringify(value) : value }}</div>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Pinia'})
  import {ref} from 'vue'
  import {usePiniaStore} from '@/stores'

  // 操作pinia
  const indexStore = usePiniaStore()
  const clearpinia = async () => {
    await indexStore.clearProfile() // 更新pinia数据
    parseAllLocalStorage() // 更新显示localStorage数据
  }
  
  let timeoutId: number | null = null;
  const handleInput = () => {
    // 清除之前的定时器
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    // 设置新的定时器
    timeoutId = window.setTimeout(() => {
      parseAllLocalStorage() // 更新显示localStorage数据
    }, 300);
  };

  // 操作localStorage
  const parsedData = ref({})
  const parseAllLocalStorage = () => {
    const allData: Record<string, any> = {}
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)
      if (key) {
        const value = localStorage.getItem(key)
        if (value !== null) {
          try {
            allData[key] = JSON.parse(value)
          } catch (e) {
            allData[key] = value // 如果不是 JSON 字符串，直接存储原始值
          }
        }
      }
    }
    parsedData.value = allData
  }
  
  parseAllLocalStorage() // 初始化显示localStorage数据
</script>

<style scoped lang="scss">
.pinia-content {
  display: flex;
  flex-direction: column;
  gap: 30px;
  
  // 卡片通用样式
  .card {
    background: white;
    border-radius: 12px;
    padding: 25px;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
   
    h3 {
      margin-top: 0;
      margin-bottom: 20px;
      font-size: 1.4rem;
      color: #333;
      text-align: center;
      position: relative;
      padding-bottom: 10px;
      
      &:after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 60px;
        height: 3px;
        background: linear-gradient(90deg, #24C8DB, #5CCEDB);
        border-radius: 3px;
      }
    }
  }
  
  // Pinia 存储区域样式
  .piniaStorage {
    .data-display {
      background: #f5f7fa;
      padding: 15px;
      border-radius: 8px;
      margin-bottom: 20px;
      font-size: 1.1rem;
      color: #333;
      text-align: center;
      min-height: 30px;
      word-break: break-all;
      border: 1px dashed #ddd;
    }
    
    .input-group {
      display: flex;
      gap: 10px;
      
      input {
        flex: 1;
        padding: 12px 15px;
        border: 1px solid #ddd;
        border-radius: 8px;
        font-size: 1rem;
        transition: all 0.3s ease;
        outline: none;
        
        &:focus {
          border-color: #24C8DB;
          box-shadow: 0 0 0 2px rgba(36, 200, 219, 0.2);
        }
        
        &::placeholder {
          color: #aaa;
        }
      }
      
      .clear-btn {
        padding: 12px 20px;
        border-radius: 8px;
        background: linear-gradient(90deg, #ff6b6b, #c03c3c);
        color: white;
        border: none;
        cursor: pointer;
        font-size: 1rem;
        font-weight: 500;
        transition: all 0.3s ease;
        
        &:hover {
          background: linear-gradient(90deg, #ff5252, #b03535);
          transform: translateY(-2px);
          box-shadow: 0 4px 10px rgba(192, 60, 60, 0.3);
        }
        
        &:active {
          transform: translateY(0);
        }
      }
    }
  }
  
  // LocalStorage 展示区域样式
  .localStorage {
    .storage-list {
      max-height: 300px;
      overflow-y: auto;
      
      &::-webkit-scrollbar {
        width: 6px;
      }
      
      &::-webkit-scrollbar-track {
        background: #f1f1f1;
        border-radius: 10px;
      }
      
      &::-webkit-scrollbar-thumb {
        background: #ccc;
        border-radius: 10px;
        
        &:hover {
          background: #aaa;
        }
      }
    }
    
    .empty-data {
      text-align: center;
      padding: 20px;
      color: #888;
      font-style: italic;
    }
    
    ul {
      list-style: none;
      padding: 0;
      margin: 0;
    }
    
    .storage-item {
      padding: 12px 15px;
      border-radius: 8px;
      background: #f8f9fa;
      margin-bottom: 10px;
      transition: all 0.2s ease;
      display: flex;
      flex-direction: column;
      gap: 5px;
      
      &:hover {
        background: #f0f2f5;
      }
      
      .storage-key {
        font-weight: 600;
        color: #24C8DB;
      }
      
      .storage-value {
        color: #555;
        word-break: break-all;
        font-family: monospace;
        background: rgba(0, 0, 0, 0.03);
        padding: 5px 8px;
        border-radius: 4px;
      }
    }
  }
}
</style>
 