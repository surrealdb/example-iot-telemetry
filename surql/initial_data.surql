CREATE site:`site-1`;
CREATE site:`site-2`;

RELATE sensor:`sensor-0`->located_at->site:`site-2`;
RELATE sensor:`sensor-1`->located_at->site:`site-1`;
RELATE sensor:`sensor-2`->located_at->site:`site-2`;
RELATE sensor:`sensor-3`->located_at->site:`site-1`;
RELATE sensor:`sensor-4`->located_at->site:`site-2`;
RELATE sensor:`sensor-5`->located_at->site:`site-1`;
RELATE sensor:`sensor-6`->located_at->site:`site-2`;
RELATE sensor:`sensor-7`->located_at->site:`site-1`;
