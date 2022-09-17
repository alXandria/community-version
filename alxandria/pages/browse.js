import Layout from '../components/layout';
import Link from 'next/link';
import { getAllDocuments } from '../db/utils'

export async function getServerSideProps(context) {

  const documents = await getAllDocuments()

  return { 
    props: { 
      documents: documents
    }
  }
  
}

export default function Browse({documents}) {

  return (
      <Layout>
          <section>
          <ul>
          { documents.map((document) => (
              <li key={document.id}>
              <Link href={`/documents/view/${document.id}`}>{document.title + " v" + document.version}</Link>
              </li>
          )) }
          </ul>
          <Link href="/documents/create">+</Link>
      </section>
    </Layout>
  ) 
}