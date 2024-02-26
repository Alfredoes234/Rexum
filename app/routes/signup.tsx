import type { MetaFunction, ActionFunctionArgs } from "@remix-run/node";
import { Form, json, redirect } from "@remix-run/react";
import { z } from "zod";

export const meta: MetaFunction = () => {
    return [
        { title: "New Remix App" },
        { name: "description", content: "Welcome to Remix!" },
    ];
};

export const signupSchema = z.object({
    name: z.string().min(1).max(8).trim(),
    email: z.string().email().trim(),
    password: z.string().min(8).max(12).trim(),
});

export async function action({
    request,
}: ActionFunctionArgs) {
    const formData = await request.formData();
    const body = Object.fromEntries(formData.entries());
    const result = await signupSchema.safeParseAsync(body);
    if (!result.success) {
        return json({ error: result.error.format() });
    }
    const content = JSON.stringify({
        "name": `${result.data.name}`,
        "email": `${result.data.email}`,
        "password": `${result.data.password}`
    })

    try {
        const response = await fetch('http://127.0.0.1:8080/api/usera', {
            method: 'POST',
            body: content,
            headers: { 'Content-Type': 'application/json' }
        });

        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }
    } catch (error) {
        console.log('There was an error', error);
    }
    

    return redirect(`/`);
}

export default function SignUp() {
    return (
        <div>
            <Form method="post">
                <input type="text" name="name" />
                <input type="text" name="email" />
                <input type="text" name="password" />
                <button type="submit">Create user</button>
            </Form>
        </div>
    )
}