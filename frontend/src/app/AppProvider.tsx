'use client';
import { createTheme, ThemeProvider } from "@mui/material";
import { PropsWithChildren } from "react";


export default function AppProvider ({ children }: PropsWithChildren) {
  const actionItemOverride = {
    boxShadow: 'var(--shadow-emphasis)',
    borderRadius: 'var(--border-radius-emphasis)',
    textTransform: 'none',
    color: 'inherit',
    fontSize: '18px',
    letterSpacing: '0',
    lineHeight: '1'
  } as const

  const darkTheme = createTheme({
    palette: {
      mode: 'dark',
    },
    components: {
      MuiButton: {
        styleOverrides: {
          root: {
            ...actionItemOverride,
            padding: '8px 16px'
          }
        }
      },
      MuiIconButton: {
        styleOverrides: {
          root: actionItemOverride
        }
      },
      MuiTouchRipple: {
        styleOverrides: {
          ripple: {
            animationDelay: '-0.1s'
          }
        }
      }
    }
  });

  return <ThemeProvider theme={darkTheme}>
    {children}
  </ThemeProvider>
}
