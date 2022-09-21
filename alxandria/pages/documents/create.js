import React from 'react';
import Layout from '../../components/layout';
import Head from 'next/head';
import DocumentForm from '../../components/documentForm'
import { useState } from 'react';
import dynamic from 'next/dynamic';
import "easymde/dist/easymde.min.css";

const SimpleMDE = dynamic(() => import("react-simplemde-editor"), { ssr: false })

export default function Document({ document }) {

  return (

    <Layout>

        <Head>
            <title>{"create:"}</title>
        </Head>

        <DocumentForm />
        
    </Layout>

  ) 
}

async function save(document) {
    const response = await fetch('/api/documents', {
        method: 'POST',
        body: JSON.stringify(document)
    });

    if (!response.ok) {
        throw new Error(response.statusText);
    }

    return await response.json();
}