'use client';
import { store } from "@/store/store";
import { createTheme, ThemeProvider } from "@mui/material";
import { PropsWithChildren } from "react";
import { Provider } from "react-redux";


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
          },
          sizeSmall: {
            padding: '6px 8px'
          }
        }
      },
      MuiIconButton: {
        styleOverrides: {
          root: actionItemOverride
        }
      },
      MuiButtonGroup: {
        styleOverrides: {
          root: {
            ...actionItemOverride,
            lightingColor: 'red'
          },
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
    <Provider store={store}>
      {children}
    </Provider>
  </ThemeProvider>
}
