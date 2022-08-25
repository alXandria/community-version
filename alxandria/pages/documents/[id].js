import React from 'react';
import Layout from '../../components/layout';
import Head from 'next/head';
import { useState } from 'react';
import { getAllDocumentIds, getDocumentData } from '../../lib/documents';
import dynamic from 'next/dynamic';
import "easymde/dist/easymde.min.css";

const SimpleMDE = dynamic(() => import("react-simplemde-editor"), { ssr: false })

export default function Document({ documentData }) {

    // Editing state variable
    const [editing, setEditing] = useState(false);

    function toggleEditing() {
        setEditing(!editing);
    }

    function computeEditingCosts(){
        return
    }

  return (

    <Layout>

        <Head>
            <title>{documentData.title}</title>
        </Head>

        <h1>{documentData.title}</h1>
        {/* Editing button */}
        <em>last edited: {documentData.date}</em> <button onClick={toggleEditing}>{editing ? 'Cancel Editing' : 'Edit'}</button>
        <br />

        {/* Read-only display */}
        <div style={{display: editing ? 'none' : 'block'}}>
            {/* Need to find safety checker? */}
            <div dangerouslySetInnerHTML={{ __html: documentData.contentHtml }}/>
            <br />
        </div>

        {/* Editing display */}
        <SimpleMDE style={{display: editing ? 'block' : 'none'}}
            value={documentData.contentMd}
            options={
                {
                    autofocus: true,
                    spellChecker: false,
                }
            }
            onChange={computeEditingCosts}
        />
        
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