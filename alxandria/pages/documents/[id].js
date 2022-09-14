import React from 'react';
import Layout from '../../components/layout';
import Head from 'next/head';
import { useState } from 'react';
import dynamic from 'next/dynamic';
import "easymde/dist/easymde.min.css";
import { getDocument } from '../../db/utils'

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
    const [editing, setEditing] = useState(false);
    const [contentMd, setContentMd] = useState(document.markdown)

    function toggleEditing() {
        setEditing(!editing);
    }

    function onChangeMd(md){
        setContentMd(md);
    }

    function save(){
        // saveDocumentData(document.id, contentMd);
        toggleEditing()
        return
    }

  return (

    <Layout>

        <Head>
            <title>{document.title}</title>
        </Head>

        <h1>{document.title + " v" + document.version}</h1>

        <button onClick={toggleEditing}>{editing ? 'Cancel Editing' : 'Edit'}</button>
        <button onClick={save}>Save</button>
        <br />

        {/* Read-only display */}
        <div style={{display: editing ? 'none' : 'block'}}>
            {/* Need to find safety checker? */}
            <div dangerouslySetInnerHTML={{ __html: document.markdown }}/>
            <br />
        </div>

        {/* Editing display */}
        <SimpleMDE style={{display: editing ? 'block' : 'none'}}
            value={contentMd}
            options={
                {
                    autofocus: true,
                    spellChecker: false,
                }
            }
            onChange={ md => onChangeMd(md) }
        />
        
    </Layout>

  ) 
}