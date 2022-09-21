import { useForm } from 'react-hook-form'
import dynamic from 'next/dynamic';
import "easymde/dist/easymde.min.css";
import { useState } from 'react';

const SimpleMDE = dynamic(() => import("react-simplemde-editor"), { ssr: false })

export default function DocumentForm() {

    const { register, handleSubmit, formState: { errors } } = useForm();
    const [markdown, setMarkdown] = useState("");

    const onChange = (value: string) => {
        setMarkdown(value);
    }

    const onSubmit = (data: any) => {
        console.log(data, markdown);
    }

    return (
        <div>
        <form onSubmit={handleSubmit(onSubmit)}>

            <input placeholder="Title" {...register("title", { required: true})} />
            <br/>
            <br />

            <SimpleMDE
                value={markdown}
                onChange={onChange}
            />

            <input type="submit" />

        </form>

        </div>

        
    )
}