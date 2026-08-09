import { createSignal, Show } from "solid-js";

interface Props {
  endpoint: string;
}

export default function ContactForm(props: Props) {
  const [isSubmitting, setIsSubmitting] = createSignal(false);
  const [error, setError] = createSignal("");
  let form: HTMLFormElement | undefined;
  let successDialog: HTMLDialogElement | undefined;

  const submitForm = async (event: SubmitEvent) => {
    event.preventDefault();
    if (!form?.reportValidity()) return;

    setIsSubmitting(true);
    setError("");

    try {
      const response = await fetch(props.endpoint, {
        method: "POST",
        body: new FormData(form),
        headers: { Accept: "application/json" },
      });

      if (!response.ok) throw new Error("Form submission failed");

      form.reset();
      successDialog?.showModal();
    } catch {
      setError("Something went wrong. Please try again.");
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <>
      <form class="contact-form" ref={(element) => (form = element)} onSubmit={submitForm}>
        <div class="form-field">
          <label for="name">Name</label>
          <input id="name" name="name" type="text" autocomplete="name" required />
        </div>
        <div class="form-field">
          <label for="email">Email</label>
          <input id="email" name="email" type="email" autocomplete="email" required />
        </div>
        <div class="form-field">
          <label for="message">Message</label>
          <textarea id="message" name="message" rows="6" required />
        </div>
        <Show when={error()}>
          <p class="form-error" role="alert">{error()}</p>
        </Show>
        <button class="submit-button" type="submit" disabled={isSubmitting()} aria-busy={isSubmitting()}>
          {isSubmitting() ? "Sending…" : "Send message"}
        </button>
      </form>

      <dialog class="success-dialog" ref={(element) => (successDialog = element)} aria-labelledby="success-title">
        <h2 id="success-title">Message sent</h2>
        <p>I’ll message back as soon as possible.</p>
        <form method="dialog">
          <button class="dialog-button" type="submit">Close</button>
        </form>
      </dialog>
    </>
  );
}
