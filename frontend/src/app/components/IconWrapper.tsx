import './IconWrapper.scss';
import { PropsWithChildren } from "react"

interface IconWrapperProps extends PropsWithChildren {
  flatShadow?: boolean
}
export default function IconWrapper ({ flatShadow, children }: IconWrapperProps) {
  return <div className={`icon-wrapper${flatShadow ? ' flat-shadow' : ''}`}>
    {children}
  </div>
}
