import './DataList.scss';
import { PropsWithChildren } from "react";

export default function DataList ({ children }: PropsWithChildren) {
  return <div className="data-list">
    {children}
  </div>
}

export function EmptyListMessage ({ list, message }: { list: unknown[]; message: string; }) {
  if (list.length > 0) return null;
  return <p className="empty-list-message">{message}</p>
}
