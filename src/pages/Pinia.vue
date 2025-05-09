<template>
  <div class="pinia-content">
      <!-- 操作pinia -->
      <div class="piniaStorage">
        <h3>pinia存储的数据：[ {{ indexStore.useName }} ]</h3>
        <input
           type="text"
          v-model="indexStore.useName"
          placeholder="请输入数据"
          placeholder-class="input-placeholder"
          @input="handleInput"
        />
        <button class="clear" @click="clearpinia">清空</button>
        <hr>
      </div>
      <!-- 操作LocalStorage -->
      <div class="LocalStorage">
        <h3>本地所有存储</h3>
        <ul>
          <li v-for="(value, key) in parsedData" :key="key">
            {{ key }}: {{ value }}
          </li>
        </ul>
      </div>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Pinia'})
  import {ref} from 'vue'
  import {usePiniaStore} from '@/stores'

  // 操作pinia
  const indexStore = usePiniaStore()
  const clearpinia = async () =>{
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
  const parsedData = ref()
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
  parsedData.value =  allData
}
parseAllLocalStorage() // 更新显示localStorage数据


</script>

<style scoped lang="scss">
.pinia-content {
  margin-top: 30px;
  // display: flex;
  // justify-content: center;
  

  .piniaStorage {
    text-align: center;
    input{
      font-size: 18px;
    }
    button {
      padding: 5px 15px;
      border-radius: 10px;
      color: rgb(255, 255, 255);
      cursor: pointer;
      font-size: 15px;
      font-weight: bold;
      background-color: #c03c3c;
    }
  }

  .LocalStorage {
    margin-top: 15px;
    h3 {
     text-align: center;
    }
    ul{
      list-style-position: inside; // 将标记放置在内容区域内
      list-style-type: decimal; // 更换小圆点
    }
  }
}
</style>
 