import { setupServer } from 'msw/node';
import { http, HttpResponse } from 'msw';

export const server = setupServer(
  http.post('https://openrouter.ai/api/v1/chat/completions', () => {
    return HttpResponse.json({
      choices: [
        {
          message: {
            content: 'OpenRouter mock not configured for this test.',
          },
        },
      ],
    });
  }),
);

export { http, HttpResponse };
