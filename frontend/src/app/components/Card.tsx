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

interface CardProps extends PropsWithChildren {
  className?: string;
}
export default function Card ({ children, className = '' }: CardProps) {
  return <div className={`card-outer ${className}`.trim()}>
    <div className="card-inner">
      {children}
    </div>
  </div>
}
