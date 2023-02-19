import React from 'react'

export const TalkRoomList: React.FC = () => {
    return (
        <div className="mt-2">
            <div className="flex flex-col -mx-4">
                <div className="relative flex flex-row items-center p-4">
                    <div className="absolute text-xs text-gray-500 right-0 top-0 mr-4 mt-3">5 min</div>
                    <div className="flex items-center justify-center h-10 w-10 rounded-full bg-pink-500 text-pink-300 font-bold flex-shrink-0">
                        T
                    </div>
                    <div className="flex flex-col flex-grow ml-3">
                        <div className="text-sm font-medium">Cuberto</div>
                        <div className="text-xs truncate w-40">Lorem ipsum dolor sit amet, consectetur adipisicing elit. Debitis, doloribus?</div>
                    </div>
                    <div className="flex-shrink-0 ml-2 self-end mb-1">
                        <span className="flex items-center justify-center h-5 w-5 bg-red-500 text-white text-xs rounded-full">5</span>
                    </div>
                </div>
                <div className="flex flex-row items-center p-4 bg-gradient-to-r from-red-100 to-transparent border-l-2 border-red-500">
                    <div className="flex items-center justify-center h-10 w-10 rounded-full bg-pink-500 text-pink-300 font-bold flex-shrink-0">
                        T
                    </div>
                    <div className="flex flex-col flex-grow ml-3">
                        <div className="flex items-center">
                            <div className="text-sm font-medium">UI Art Design</div>
                            <div className="h-2 w-2 rounded-full bg-green-500 ml-2" />
                        </div>
                        <div className="text-xs truncate w-40">Lorem ipsum dolor sit amet, consectetur adipisicing elit. Debitis, doloribus?</div>
                    </div>
                </div>
            </div>
        </div>
    )
}
