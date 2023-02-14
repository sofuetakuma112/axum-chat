import React from 'react'
import classnames from "classnames"

type Props = {
    name: string,
    message: string
    isMine: boolean
}

export const Message: React.FC<Props> = ({ name, message, isMine }) => {
    return (
        <div className={classnames("p-3 rounded-lg", { "col-start-1 col-end-8": !isMine }, { "col-start-6 col-end-13": isMine })}>
            <div className={classnames("flex", { "flex-row items-center": !isMine }, { "items-center justify-start flex-row-reverse": isMine })}>
                <div className="flex items-center justify-center h-10 w-10 rounded-full bg-indigo-500 flex-shrink-0">
                    {name}
                </div>
                <div className={classnames("relative text-sm py-2 px-4 shadow rounded-xl", { "ml-3 bg-white": !isMine }, { "mr-3 bg-indigo-100": isMine })}>
                    <div>{message}</div>
                </div>
            </div>
        </div>
    )
}
