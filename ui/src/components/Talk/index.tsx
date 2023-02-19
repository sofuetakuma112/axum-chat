import React from 'react'
import { Message } from '../Message'

type Props = {
    messages: Message[]
}

export type Message = {
    name: string,
    message: string,
    isMine: boolean
}

export const Talk: React.FC<Props> = ({ messages }) => {
    return (
        <div className="h-full overflow-hidden py-4">
            <div className="h-full overflow-y-auto">
                <div className="grid grid-cols-12 gap-y-2">
                    {messages.map((message, i) => <Message key={i} {...message} />)}
                </div>
            </div>
        </div>
    )
}
