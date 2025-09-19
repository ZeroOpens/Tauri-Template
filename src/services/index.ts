import axiosHttp from '@/utils/http'
import { type DogResponse } from '@/types/index'

// 无跨域问题
/**
 * 随机狗狗图片API
 * @returns 随机获取狗狗图片
 */
export const getDogAPI = (): Promise<DogResponse> => {
  return axiosHttp.get('/api/breeds/image/random');
};


// 有跨域问题，需在 vite.config.ts 添加代理
/**
 * 随机语句API
 * @returns 随机获取一句话
 */
export const getWordAPI = (): Promise<string> => {
  return axiosHttp.get('/api-ziyi/anime-dailytxt/', {
    baseURL: ''  // 需设置 baseURL 为空
  });
};
