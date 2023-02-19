import React, { useCallback, useRef, useState } from 'react'
import { MenuBar } from '../components/MenuBar'
import { SideBar } from '../components/SideBar'
import { Talk } from '../components/Talk'
import { TalkForm } from '../components/Talk/TalkForm'

export const Chat: React.FC = () => {
    const [connectionClosed, setConnectionClosed] = useState(false)

    const websocket = useRef<WebSocket>()

    const handleClickButton = useCallback(() => {
        websocket.current = new WebSocket("ws://localhost:3000/websocket");

        websocket.current.onopen = function () {
            console.log("connection opened");
        };

        websocket.current.onclose = function () {
            console.log("connection closed");
            setConnectionClosed(true)
        };

        websocket.current.onmessage = function (e) {
            // setMessages(oldMessages => [...oldMessages, newMessage])
        };
    }, [])

    const handleClickSubmitButton = (messageInput: string) => {
        if (messageInput) {
            websocket.current?.send(messageInput);
        }
    }

    return (
        <div className="flex flex-row h-screen antialiased text-gray-800">
            <div className="flex flex-row w-96 flex-shrink-0 bg-gray-100 pr-4">
                <MenuBar />
                <SideBar />
            </div>
            <div className="flex flex-col h-full w-full bg-white">
                {/* <TalkRoomInfo /> */}
                <Talk messages={[]} />
                <TalkForm handleClickSubmitButton={handleClickSubmitButton} />
            </div>
        </div>
    )
}
