import Layout from '../components/layout';
import Link from 'next/link';
import { getSortedDocumentsData } from '../lib/documents';

export async function getServerSideProps(context) {
    const allDocumentsData = getSortedDocumentsData();
    return {
      props: {
        allDocumentsData,
      },
    };
  }

export default function Document({allDocumentsData}) {

    return (
        <Layout>
            <section>
            <ul>
            {allDocumentsData.map(({ id, date, title }) => (
                <li key={id}>
                <Link href={`/documents/${id}`}>{title}</Link>
                <br />
                <em>last edited: {date}</em>
                </li>
            ))}
            </ul>
        </section>
      </Layout>
    ) 
  }