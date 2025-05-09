import {defineStore} from 'pinia'
import {ref} from 'vue'

export const usePiniaStore = defineStore(
  'pinia', 
  () => {
        // 数据信息
        const useName = ref<any>()

        // 保存数据信息
        const setProfile = (val: any) => {
          useName.value = val
        }
    
        // 清理数据信息
        const clearProfile = () => {
          useName.value = undefined
        }
    
        return { useName, setProfile, clearProfile, }
  },
  {
    persist: true, // 开启持久化
  },
)
