
import streamlit as st

def load_theme():
    if "theme_mode" not in st.session_state:
        st.session_state.theme_mode = "Dunkel"

    theme = {
        "Dunkel": {
            "bg": "#111111",
            "text": "#EEEEEE",
            "accent": "#00FFC2",
            "box": "#222222"
        },
        "Hell": {
            "bg": "#F9F9F9",
            "text": "#222222",
            "accent": "#0055FF",
            "box": "#FFFFFF"
        }
    }

    st.session_state.theme_mode = st.selectbox("Theme auswählen", ["Dunkel", "Hell"], index=["Dunkel", "Hell"].index(st.session_state.theme_mode))
    return theme[st.session_state.theme_mode]

def styled_box(title, content, theme):
    st.markdown(f'''
    <div style="
        background-color: {theme['box']};
        border-left: 5px solid {theme['accent']};
        padding: 1rem;
        margin-bottom: 1rem;
        color: {theme['text']};
        border-radius: 8px;">
        <h4 style="margin-top:0;">{title}</h4>
        <p>{content}</p>
    </div>
    ''', unsafe_allow_html=True)
