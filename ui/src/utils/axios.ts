import axios from 'axios'

axios.defaults.baseURL = 'http://localhost:3000/api/'

// フロントエンドとバックエンドのポート番号が異なるクロスオリジンの環境でCookiesを扱えるようになります。
axios.defaults.withCredentials = true

// TODO: リフレッシュトークンを実装したらコメントアウトする
// axios.interceptors.response.use(
//   (response) => response,
//   (error) => {
//     // axiosの設定情報をoriginalConfigとして保存
//     const originalConfig = error.config
//     if (
//       error.response.status === 401 &&
//       originalConfig.url === '/auth/refresh_token'
//     ) {
//       // リフレッシュトークンの検証に失敗している
//       return Promise.reject(error)
//     }

//     if (error.response.status === 401 && !originalConfig._retry) {
//       // アクセストークンを利用した最初のエラーである
//       originalConfig._retry = true
//       return axios.get('/auth/refresh_token').then(() => {
//         // リフレッシュトークンの検証に成功したので、
//         // 新たに発行されたアクセストークンを利用して再度リクエストを送信
//         return axios(originalConfig)
//       })
//     }
//     return Promise.reject(error)
//   }
// )

export const fetcher = (url: string) => axios.get(url).then((res) => res.data)

export default axios
