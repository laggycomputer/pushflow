import './DataRow.scss';
import { PropsWithChildren, ReactNode } from "react";

interface DataRowInformationProps extends PropsWithChildren {
  title: ReactNode;
}
export function DataRowInformation ({ title, children }: DataRowInformationProps) {
  return <div className="row-information">
    <b className="row-title">{title}</b>
    <p className="row-stats">{children}</p>
  </div>
}

interface DataRowStatItem {
  icon: ReactNode;
  text: ReactNode;
}
export function DataRowStatItem ({ icon, text }: DataRowStatItem) {
  return <span className="row-stat">
    {icon}
    {text}
  </span>
}

export default function DataRow ({ children }: PropsWithChildren) {
  return <div className="data-row">{children}</div>
}
