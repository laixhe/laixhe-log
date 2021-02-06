package main

import (
	"bytes"
	"fmt"
	"image"
	"image/color"
	"io/ioutil"

	"github.com/disintegration/imaging"
)

func main() {

	//读取本地文件，本地文件尺寸300*400
	imgData, err := ioutil.ReadFile("1.png")
	if err != nil {
		fmt.Println("ReadFile:", err)
		return
	}

	buf := bytes.NewBuffer(imgData)
	img, err := imaging.Decode(buf)
	if err != nil {
		fmt.Println("Decode:", err)
		return
	}

	// 可以使用这个替换成上面的
	// imaging.Open("1.png")

	width := img.Bounds().Dx()
	height := img.Bounds().Dy()

	var dst *image.NRGBA
	if width >= height {
		// 生成缩略图，尺寸宽200,高传0表示等比例放缩
		dst = imaging.Resize(img, 200, 0, imaging.Lanczos)
	} else {
		// 生成缩略图，尺寸宽传0表示等比例放缩,高传300
		dst = imaging.Resize(img, 0, 300, imaging.Lanczos)
	}

	err = imaging.Save(dst, "2.png")
	if err != nil {
		fmt.Println("Save2:", err)
	}

	// 尽量将图片填充到 200x200 比例中，并不拉伸
	dst = imaging.Fill(img, 200, 200, imaging.Center, imaging.Lanczos)
	err = imaging.Save(dst, "3.png")
	if err != nil {
		fmt.Println("Save3:", err)
	}

	// 生成固定大小
	fileFormat, _ := imaging.FormatFromFilename("1.png")
	dst = imaging.Resize(img, 200, 200, imaging.Lanczos)
	buf1 := bytes.NewBuffer(nil)
	err = imaging.Encode(buf1, dst, fileFormat)
	if err != nil {
		fmt.Println("Encode:", err)
		return
	}

	err = ioutil.WriteFile("4.png", buf1.Bytes(), 0755)
	if err != nil {
		fmt.Println("WriteFile:", err)
		return
	}

	// 从中心点直接裁剪到 200x200 大小
	dst = imaging.CropAnchor(img, 200, 200, imaging.Center)
	err = imaging.Save(dst, "5.png")
	if err != nil {
		fmt.Println("Save5:", err)
	}

	// 对图片进行模糊
	dst = imaging.Blur(img, 5)
	err = imaging.Save(dst, "6.png")
	if err != nil {
		fmt.Println("Save6:", err)
	}

	// 创建一个新的图像，并粘贴四个产生的图像到其中
	dst1 := imaging.New(400, 400, color.NRGBA{0, 0, 0, 0})
	dst1 = imaging.Paste(dst1, dst, image.Pt(0, 0))                                                                         // 模糊
	dst1 = imaging.Paste(dst1, imaging.Grayscale(img), image.Pt(0, 200))                                                    // 灰度
	dst1 = imaging.Paste(dst1, imaging.Invert(img), image.Pt(200, 0))                                                       // 倒置
	dst1 = imaging.Paste(dst1, imaging.Convolve3x3(img, [9]float64{-1, -1, 0, -1, 1, 1, 0, 1, 1}, nil), image.Pt(200, 200)) // 浮雕

	err = imaging.Save(dst1, "7.png")
	if err != nil {
		fmt.Println("Save7:", err)
	}

}
