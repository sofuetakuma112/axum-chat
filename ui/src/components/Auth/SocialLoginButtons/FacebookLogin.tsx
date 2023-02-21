import React from 'react'

export const FacebookLogin: React.FC = () => {
  return (
    <button className="relative mt-6 border rounded-md py-2 text-sm text-gray-800 bg-gray-100 hover:bg-gray-200">
      <span className="absolute left-0 top-0 flex items-center justify-center h-full w-10 text-blue-500">
        <i className="fab fa-facebook-f" />
      </span>
      <span>Login with Facebook</span>
    </button>
  )
}
