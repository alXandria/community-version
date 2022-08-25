import Layout from '../../components/layout';
import { getAllDocumentIds, getDocumentData } from '../../lib/documents';

export default function Document({ documentData }) {

  return (

    <Layout>
        <b>{documentData.title}</b>
        <br />
        <em>edited: {documentData.date}</em>
        <br />
        <div dangerouslySetInnerHTML={{ __html: documentData.contentHtml }}/>
    </Layout>

  ) 
}

export async function getServerSidePaths(){

    const paths = getAllDocumentIds();

    return {
        paths,
        fallback: false,
    };
}

export async function getServerSideProps({ params }){

    const documentData = await getDocumentData(params.id);
    
    return {
        props: {
            documentData,
        },
    }
}