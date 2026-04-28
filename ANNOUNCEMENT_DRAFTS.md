# 3DNTerminal v0.3.6 — Community Announcement Drafts

This document contains pre-written messages you can use to announce 3DNTerminal to the COSMIC and Rust communities.

---

## 1. GitHub Comment (Reference Implementation)
**Target**: [pop-os/cosmic-settings#923](https://github.com/pop-os/cosmic-settings/issues/923) or [pop-os/libcosmic#1116](https://github.com/pop-os/libcosmic/issues/1116)

**Message**:
> Hi there! I've been following the discussions on native COSMIC accessibility and smooth animations. I’ve implemented a reference terminal called **3DNTerminal** that specifically focuses on these areas using `libcosmic`. 
> 
> It currently features:
> - **Tremor Compensation**: A low-pass filter for smooth cursor interaction.
> - **Real-time Color Filters**: Protanopia, Deuteranopia, and Tritanopia transformations.
> - **Magnetic Focus**: A physics-based UI that reacts organically to the cursor.
>
> Feel free to use the source code as a reference for the official COSMIC accessibility implementations! 
> Repo: https://github.com/doko1975/3DNTerminal

---

## 2. Reddit Post (Showcase)
**Target**: `r/pop_os`, `r/cosmic_desktop`, or `r/rust`

**Title**: [Showcase] 3DNTerminal — A Cyberpunk Hologram Terminal with Advanced Accessibility for COSMIC Desktop

**Content**:
> Hey everyone! I wanted to share a project I’ve been working on: **3DNTerminal**.
>
> It’s a floating, translucent terminal built with Rust and libcosmic. While the "Cyberpunk" look was the starting point, I’ve pivoted to making it a showcase for **advanced accessibility** and **physics-based UI** in the COSMIC ecosystem.
>
> **Key Features:**
> - 🟢 **Tremor Compensation**: Smooths out shaky cursor movements for better precision.
> - 🌈 **Vision Filters**: Built-in filters for different types of color blindness.
> - 🧲 **Magnetic UI**: The window reacts to your cursor proximity, giving it an "alive" feel.
> - 🧩 **Modular Skill System**: Easy to extend with new themes or physics modules.
>
> It’s open-source (Apache 2.0) and I’d love for it to serve as a reference for other libcosmic developers looking into A11Y and smooth animations.
> 
> Check it out here: https://github.com/doko1975/3DNTerminal

---

## 3. Discord / Mattermost (Quick Chat)
**Target**: COSMIC Desktop Dev Channels

**Message**:
> Just released v0.3.6 of 3DNTerminal! 🦀 It’s a libcosmic-based terminal that I’ve turned into a playground for **Accessibility (A11Y)** and **Physics-based animations**. If anyone is looking for a reference on how to implement tremor-damping or real-time color filters in a Rust/Iced app, feel free to check the source! 🚀 https://github.com/doko1975/3DNTerminal
