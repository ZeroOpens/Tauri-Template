//定义一个接口，用于限制对象的具体属性
export interface userType {
  id: string,
  userName: string,
  password: number
}

// 自定义类型
export type usersType = user[]

