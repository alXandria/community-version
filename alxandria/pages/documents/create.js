import React from 'react';
import Layout from '../../components/layout';
import Head from 'next/head';
import { useState } from 'react';
import dynamic from 'next/dynamic';
import "easymde/dist/easymde.min.css";

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
            <title>{"create:"}</title>
        </Head>

        <input placeholder='title' type={"text"} id={"document-title"}></input>
        <br />
        <br />

        <SimpleMDE id="document-markdown"
            value={markdown}
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