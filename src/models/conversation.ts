interface Message {
    id: number;
    assistant_uuid: string;
    model: string;
    created_at: Date;
    content: string;
    role: string;
    total_duration: number;
    load_duration: number;
    prompt_eval_count: number;
    prompt_eval_duration: number;
    eval_count: number;
    eval_duration: number;
}