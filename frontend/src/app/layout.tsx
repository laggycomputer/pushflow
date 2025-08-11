import type { Metadata } from "next";
import { Geist, Geist_Mono, Roboto } from "next/font/google";
import "./globals.scss";
import Header from "./components/Header";
import AppProvider from "./AppProvider";
import { getUser } from "@/helpers/server";

const geistSans = Geist({
  variable: '--font-geist-sans',
  subsets: ['latin'],
});

const geistMono = Geist_Mono({
  variable: '--font-geist-mono',
  subsets: ['latin'],
});

const roboto = Roboto({
  variable: '--font-roboto',
  subsets: ['latin']
})

export const metadata: Metadata = {
  title: "PushFlow",
  description: "Send WebPush Notifications without the hassle of configuration!",
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const user = await getUser()

  return (
    <html lang="en">
      <AppProvider>
        <body className={`${geistSans.variable} ${geistMono.variable} ${roboto.variable}`}>
          <Header user={user} />
          {children}
        </body>
      </AppProvider>
    </html>
  );
}
