import React, { useState } from 'react'
import { User } from '../../hooks/useAuth'
import { Input } from '../Forms/Input'
import { Header, Tab } from './Header'
import { List } from './List'
import { useForm, SubmitHandler } from 'react-hook-form'
import { useAuthContext } from '../../context/AuthContext'
import { fetcher } from '../../utils/axios'
import useSWR from 'swr'
import { Modal } from '../Modal'
import { ProfileCard } from './ProfileCard'

type Props = {
  friends: User[]
}

type Inputs = {
  searchInput: string
}

export const Friends: React.FC<Props> = ({ friends }) => {
  const [currentTab, setCurrentTab] = useState<Tab>('List')
  const [isModalOpen, setIsModalOpen] = useState(false)

  const { user } = useAuthContext()

  const {
    watch,
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<Inputs>()
  const onSubmit: SubmitHandler<Inputs> = (data) => {}

  const { data: followeesData, isLoading: isLoadingFollowees } = useSWR(
    currentTab === 'List' ? `/users/${user?.id}/followees` : null,
    fetcher
  )

  const { data: usersData, isLoading: isLoadingUsers } = useSWR(
    currentTab === 'Search' && watch('searchInput')
      ? `/users?user_id=${watch('searchInput')}`
      : null,
    fetcher
  )

  const handleClickHeaderIcon = (tab: Tab) => {
    setCurrentTab(tab)
  }

  // if (currentTab === 'List' && isLoadingFollowees) {
  //   return <></>
  // } else if (currentTab === 'Search' && isLoadingUsers) {
  //   return <></>
  // }

  const showUserDetail = (id: number) => {
    setIsModalOpen(true)
    console.log(id)
  }

  return (
    <>
      <Modal isOpen={isModalOpen} handleClose={() => setIsModalOpen(false)}>
        <ProfileCard imageUrl={'https://source.unsplash.com/150x150/?portrait?3'} name={'Leroy Jenkins'} />
      </Modal>
      <div className="flex flex-col w-full h-full px-4 pt-4">
        <Header handleClickHeaderIcon={handleClickHeaderIcon} />
        <div className="py-4">
          {currentTab === 'List' && (
            <>
              <Input placeholder="名前で検索" {...register('searchInput')} />
              <List
                users={followeesData?.followees || []}
                handleClickFriend={showUserDetail}
              />
            </>
          )}
          {currentTab === 'Search' && (
            <>
              <Input placeholder="IDで検索" {...register('searchInput')} />
              <List
                users={usersData?.users || []}
                handleClickFriend={showUserDetail}
              />
            </>
          )}
        </div>
      </div>
    </>
  )
}
