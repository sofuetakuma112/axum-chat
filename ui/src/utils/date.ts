import {
  differenceInDays,
  differenceInHours,
  differenceInMinutes,
  differenceInMonths,
  differenceInSeconds,
  differenceInWeeks,
  differenceInYears
} from 'date-fns'

export const diffTime = (base: Date, target: Date) => {
  const diffInSecs = differenceInSeconds(base, target)
  if (diffInSecs < 60) {
    return `${diffInSecs}秒前`
  }

  const diffInMins = differenceInMinutes(base, target)
  if (diffInMins < 60) {
    return `${diffInMins}分前`
  }

  const diffInHours = differenceInHours(base, target)
  if (diffInHours < 24) {
    return `${diffInHours}時間前`
  }

  const diffInDays = differenceInDays(base, target)
  if (diffInDays < 7) {
    return `${diffInDays}日前`
  }

  const diffInWeeks = differenceInWeeks(base, target)
  if (diffInWeeks < 4) {
    return `${diffInWeeks}週間前`
  }

  const diffInMonths = differenceInMonths(base, target)
  // 4週間前でも 0ヶ月前と表示されるため、条件を足して絞り込む
  if (diffInWeeks >= 4 && diffInMonths < 2) {
    return `1ヶ月前`
  } else if (diffInMonths < 12) {
    return `${diffInMonths}ヶ月前`
  }

  const diffInYears = differenceInYears(base, target)

  return `${diffInYears}年前`
}
