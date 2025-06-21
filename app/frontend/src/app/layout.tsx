import type { Metadata } from "next";
import "./globals.css";
import Header from "../components/Header";
import { ReduxProvider } from "../lib/providers";

export const metadata: Metadata = {
  title: "ARTIFOX - Minimalist Furniture",
  description: "Minimalist products for your home and office",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <ReduxProvider>
          <Header />
          {children}
        </ReduxProvider>
      </body>
    </html>
  );
}
