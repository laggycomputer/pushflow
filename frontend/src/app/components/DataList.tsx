import './DataList.scss';
import { PropsWithChildren } from "react";

export default function DataList ({ children }: PropsWithChildren) {
  return <div className="data-list">
    {children}
  </div>
}
