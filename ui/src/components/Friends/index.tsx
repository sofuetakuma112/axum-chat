import React, { useState } from 'react'
import { User } from '../../hooks/useAuth'
import { Input } from '../Forms/Input'
import { Header, Tab } from './Header'
import { List } from './List'
import { useForm, SubmitHandler } from 'react-hook-form'

type Props = {
  friends: User[]
}

type Inputs = {
  searchInput: string
}

export const Friends: React.FC<Props> = ({ friends }) => {
  const [currentTab, setCurrentTab] = useState<Tab>('List')

  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<Inputs>()
  const onSubmit: SubmitHandler<Inputs> = (data) => {}

  const handleClickHeaderIcon = (tab: Tab) => {
    setCurrentTab(tab)
  }

  return (
    <div className="flex flex-col w-full h-full px-4 pt-4">
      <Header handleClickHeaderIcon={handleClickHeaderIcon} />
      {currentTab === 'List' && (
        <div className="py-4">
          <Input placeholder="名前で検索" />
          <List
            friends={friends}
            handleClickFriend={function (id: number): void {
              throw new Error('Function not implemented.')
            }}
          />
        </div>
      )}
      {currentTab === 'Search' && (
        <div className="py-4">
          <Input placeholder="IDで検索" />
          <List
            friends={friends}
            handleClickFriend={function (id: number): void {
              throw new Error('Function not implemented.')
            }}
          />
        </div>
      )}
    </div>
  )
}
