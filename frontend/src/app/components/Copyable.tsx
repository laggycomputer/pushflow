import { IconButton } from '@mui/material';
import './Copyable.scss';
import { PropsWithChildren } from "react";

import ContentCopyIcon from '@mui/icons-material/ContentCopy';

interface CopyableProps extends PropsWithChildren {
  multiline?: boolean;
  children: string;
}

export default function Copyable ({ children, multiline }: CopyableProps) {
  const copyText = () => {
    navigator.clipboard.writeText(children)
  }

  return <div className={`theme-emphasis-box copy-box${multiline ? ' multiline' : ''}`}>
    <pre><code>
      {children}
    </code></pre>
    <IconButton
      aria-label="Copy API Key"
      onClick={copyText}
      className="theme-none"
    >
      <ContentCopyIcon />
    </IconButton>
  </div>
}
