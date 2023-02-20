import React from 'react'
import { User } from '../../hooks/useAuth'
import { Header } from './Header'
import { List } from './List'

type Props = {
  friends: User[]
}

export const Friends: React.FC<Props> = ({ friends }) => {
  return (
    <div className="flex flex-col w-full h-full px-4 pt-4">
      <Header />
      <List
        friends={friends}
        handleClickFriend={function (id: number): void {
          throw new Error('Function not implemented.')
        }}
      />
    </div>
  )
}
