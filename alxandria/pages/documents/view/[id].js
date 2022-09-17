import React from 'react';
import Layout from '../../../components/layout';
import Link from 'next/link';
import Head from 'next/head';
import dynamic from 'next/dynamic';
import "easymde/dist/easymde.min.css";
import { getDocument } from '../../../db/utils'

export async function getServerSideProps({ params }) {
        
    return {
        props: {
            document: await getDocument(params.id)
        },
    }
}

const SimpleMDE = dynamic(() => import("react-simplemde-editor"), { ssr: false })

export default function Document({ document }) {

  return (

    <Layout>

        <Head>
            <title>{document.title}</title>
        </Head>

        <h1>{document.title}</h1>

        <p><em>{"viewing: v" + document.version}</em> <Link href={"/documents/edit/" + document.id}>{" Edit"}</Link></p>
        <p>other versions:</p>
        
        <div dangerouslySetInnerHTML={{ __html: document.markdown }}/>
        
    </Layout>

  ) 
}