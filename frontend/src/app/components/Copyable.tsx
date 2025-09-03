import { IconButton } from '@mui/material';
import './Copyable.scss';
import { PropsWithChildren } from "react";
import Highlight from 'react-highlight'
import 'highlight.js/styles/github-dark-dimmed.css'

import ContentCopyIcon from '@mui/icons-material/ContentCopy';

interface CopyableProps extends PropsWithChildren {
  multiline?: boolean;
  children: string;
  lang?: string
}

export default function Copyable ({ children, multiline, lang = 'typescript' }: CopyableProps) {
  const copyText = () => {
    navigator.clipboard.writeText(children)
  }

  return <div className={`theme-emphasis-box copy-box${multiline ? ' multiline' : ''}`}>
    <Highlight className={lang}>
      {children}
    </Highlight>
    <IconButton
      aria-label="Copy API Key"
      onClick={copyText}
      className="theme-none"
    >
      <ContentCopyIcon />
    </IconButton>
  </div>
}
