import React, { useCallback, useRef, useState } from 'react'
import { FooterMenuBar, Menu } from '../components/FooterMenuBar'
import { TalkRooms } from '../components/TalkRooms'
import { Message, Talk } from '../components/Talk'
import { TalkForm } from '../components/Talk/TalkForm'
import { Room } from '../types/type'
import { Friends } from '../components/Friends'
import { Setting } from '../components/Setting'
import { fetcher } from '../utils/axios'
import { useAuthContext } from '../context/AuthContext'
import useSWR from 'swr'

export const ChatForMobile: React.FC = () => {
  const { user } = useAuthContext()

  const [connectionClosed, setConnectionClosed] = useState(false)
  const [currentRoomId, setCurrentRoomId] = useState(-1)
  const [messages, setMessages] = useState<Message[]>([])
  const [currentMenu, setCurrentMenu] = useState<Menu>('TalkRoomList')

  const {
    data,
    error: roomsError,
    isLoading: roomsIsLoading
  } = useSWR(`/users/${user?.id}/rooms`, fetcher)

  const websocket = useRef<WebSocket>()

  const handleClickButton = useCallback(() => {
    websocket.current = new WebSocket('ws://localhost:3000/websocket')

    websocket.current.onopen = function () {
      console.log('connection opened')
    }

    websocket.current.onclose = function () {
      console.log('connection closed')
      setConnectionClosed(true)
    }

    websocket.current.onmessage = function (e) {
      // setMessages(oldMessages => [...oldMessages, newMessage])
    }
  }, [])

  const handleClickSubmitButton = (messageInput: string) => {
    if (messageInput) {
      websocket.current?.send(messageInput)
    }
  }

  if (roomsIsLoading) {
    return <></>
  }

  return (
    <div className="flex flex-row h-screen antialiased text-gray-800">
      {currentRoomId === -1 ? (
        <div className="w-full flex-shrink-0 bg-gray-100">
          <div className="h-[calc(100%_-_58px)] overflow-y-scroll">
            {currentMenu === 'FriendList' && <Friends friends={[]} />}
            {currentMenu === 'TalkRoomList' && (
              <TalkRooms
                totalUnreadMessageCount={0}
                rooms={data?.rooms}
                currentRoomId={currentRoomId}
              />
            )}
            {currentMenu === 'Settings' && <Setting />}
          </div>
          <FooterMenuBar
            currentMenu={currentMenu}
            handleClickMenu={setCurrentMenu}
          />
        </div>
      ) : (
        <div className="flex flex-col h-full w-full bg-white">
          <Talk messages={messages} />
          <TalkForm handleClickSubmitButton={handleClickSubmitButton} />
        </div>
      )}
    </div>
  )
}
