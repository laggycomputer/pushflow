import { PropsWithChildren } from 'react'
import './Card.scss'

interface CardHeaderProps extends PropsWithChildren {
  text: string;
}
export function CardHeader ({ text, children }: CardHeaderProps) {
  return <div className="card-header">
    <h2>{text}</h2>
    {children}
  </div>
}

export default function Card ({ children }: PropsWithChildren) {
  return <div className="card-outer">
    <div className="card-inner">
      {children}
    </div>
  </div>
}
