export type Room = {
  id: number
  name: string
  lastMessage: string
  unreadMessageCount: number
  lastMessageTimestamp: Date
}

export type User = {
  id: number
  name: string
  avatarImageUrl: string
}
