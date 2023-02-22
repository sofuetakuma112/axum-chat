import React from 'react'
import { UserPlusIcon } from '../Icons/UserPlusIcon'

type Props = {
    imageUrl: string
    name: string
}

export const ProfileCard: React.FC<Props> = ({ imageUrl, name }) => {
  return (
    <div>
      <img
        src={imageUrl}
        alt=""
        className="w-32 h-32 mx-auto rounded-full dark:bg-gray-500 aspect-square"
      />
      <div className="space-y-4 text-center divide-y divide-gray-700">
        <div className="my-2 space-y-1">
          <h2 className="text-xl font-semibold sm:text-2xl">{name}</h2>
          {/* <p className="px-5 text-xs sm:text-base dark:text-gray-400">
                Full-stack developer
              </p> */}
        </div>
        <div className="flex justify-center pt-2 space-x-4 align-center">
          <button className="p-2 rounded-md dark:text-gray-100 hover:dark:text-violet-400">
            <UserPlusIcon size="lg" />
            <span>追加</span>
          </button>
        </div>
      </div>
    </div>
  )
}
