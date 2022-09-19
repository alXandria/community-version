import Head from 'next/head';
import styles from './layout.module.css';
import utilStyles from '../styles/utils.module.css';
import Nav from './nav.js'

export const siteTitle = 'AlXandria';

export default function Layout({ children, home }) {
  return (

    <div>

      <Nav />

      <div className={styles.container}>
        <Head>
          <title>{siteTitle}</title>
          <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"></link>
          <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png"></link>
          <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png"></link>
          <link rel="manifest" href="/site.webmanifest"></link>
          <meta
            name="AlXandria"
            content="The Decentralized Library"
          />
          <meta name="og:title" content={siteTitle} />
          <meta name="twitter:card" content="summary_large_image" />
        </Head>

        <main>
          {children}
        </main>

      </div>
      
    </div>
  );
}