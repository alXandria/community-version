import React from 'react';
import Layout from '../../../components/layout';
import Head from 'next/head';
import Link from 'next/link';
import { useState } from 'react';
import dynamic from 'next/dynamic';
import "easymde/dist/easymde.min.css";
import { getDocument } from '../../../db/utils';

export async function getServerSideProps({ params }) {
        
    return {
        props: {
            document: await getDocument(params.id)
        },
    }
}

const SimpleMDE = dynamic(() => import("react-simplemde-editor"), { ssr: false })

export default function Document({ document }) {

    // Editing state variable
    const [markdown, setMarkdown] = useState("");

    function save() {

        return
    }

  return (

    <Layout>

        <Head>
            <title>{"edit:" + document.title}</title>
        </Head>

        <h1>{ document.title }</h1>

        <p><em>{"editing: v" + document.version}</em> <Link href={"/documents/view/" + document.id}>{" View"}</Link></p>

        <SimpleMDE id="document-markdown"
            value={document.markdown}
            options={
                {
                    spellChecker: false,
                }
            }
        />

        <button onClick={save}>Save</button>
        
    </Layout>

  ) 
}