import tkinter

import xmlrpc.client

WIDTH, HEIGHT = 800, 600
SCROLL_STEP = 10


class Browser:
    def __init__(self):
        self.scroll = 0
        self.width = WIDTH
        self.height = HEIGHT
        self.window = tkinter.Tk()
        self.canvas = tkinter.Canvas(
            self.window,
            width=self.width,
            height=self.height,
            highlightthickness=0
        )
        self.canvas.pack(fill=tkinter.BOTH, expand=True)
        self.window.bind("<Down>", self.scrolldown)
        self.window.bind("<Up>", self.scrollup)
        self.window.bind("<MouseWheel>", self.mouse_wheel)
        self.window.bind("<Configure>", self.configure)
        self.layout = []
        emoji_map = {
            "\U0001F600": "1F600_color.png"
        }
        self.image_map = {

        }
        for c in emoji_map:
            self.image_map[c] = tkinter.PhotoImage(file=emoji_map[c])

    def resize_canvas(self):
        self.canvas.config(width=self.width, height=self.height)
        self.compute_layout()
        self.render()

    def load(self, url):
        proxy = xmlrpc.client.ServerProxy("http://127.0.0.1:8080")
        params = {"url": url}
        response = proxy.load_url(params)
        self.layout = response['layout']
        self.body = response['body']
        self.render()

    def compute_layout(self):
        proxy = xmlrpc.client.ServerProxy("http://127.0.0.1:8080")
        params = {"body": self.body,
                  "width": self.width, "height": self.height}
        response = proxy.relayout(params)
        self.body = response['body']
        self.layout = response['layout']

    def render(self):
        self.canvas.delete("all")
        for (cursor_x, cursor_y, c) in self.layout:
            if c == "\U0001F600":
                loaded_image = self.image_map[c]
                self.canvas.create_image(
                    cursor_x, cursor_y - self.scroll, image=loaded_image)
            else:
                self.canvas.create_text(
                    cursor_x, cursor_y - self.scroll, text=c)

    def scrolldown(self, e):
        self.scroll += SCROLL_STEP * 3
        if self.scroll < 0:
            self.scroll = 0
        self.render()

    def scrollup(self, e):
        self.scroll -= SCROLL_STEP * 3
        if self.scroll < 0:
            self.scroll = 0
        self.render()

    def mouse_wheel(self, e):
        # down
        if e.delta == -120:
            self.scrollup(e)
        else:
            # up
            self.scrolldown(e)

    def configure(self, e):
        width = e.width
        height = e.height
        if width != self.width or height != self.height:
            self.width = width
            self.height = height
            self.resize_canvas()


if __name__ == "__main__":
    import sys
    print("starting up")
    Browser().load(sys.argv[1])
    tkinter.mainloop()
